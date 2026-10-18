import { describe, expect, it } from 'vitest'
import { applyNfoLayout, normalizeNfoOptions, createNfoOptions, nfoOptionsError, previewNfoTemplate } from './nfo'
import { cloneWorkflow, createWorkflow, workflowError } from './downloadWorkflow'
import { previewTemplate, validateNamingTemplate } from '../stores/settings'

describe('NFO configuration', () => {
  it('backfills old presets and keeps task copies independent', () => {
    const old = createWorkflow()
    delete old.nfo_options
    const copy = cloneWorkflow(old)
    expect(copy.nfo_options).toEqual(createNfoOptions())
    copy.nfo_options!.custom_tags = '收藏'
    expect(old.nfo_options).toBeUndefined()
    expect(cloneWorkflow(copy).nfo_options!.custom_tags).toBe('收藏')
  })
  it('validates templates only when enabled and previews known variables', () => {
    const flow = createWorkflow()
    flow.nfo_options!.title_template = '{unknown}'
    expect(workflowError(flow)).toBeNull()
    flow.nfo = true
    expect(workflowError(flow)).toContain('NFO')
    for (const title_template of ['{title', '{title}}', '{}', '']) {
      expect(nfoOptionsError({ ...createNfoOptions(), title_template })).not.toBeNull()
    }
    expect(previewNfoTemplate('{video_title} - {part_title}')).toBe('示例视频 - 开场')
    expect(nfoOptionsError(createNfoOptions())).toBeNull()
  })
  it('supports advanced layouts, safe formats and partial old configuration', () => {
    const old = normalizeNfoOptions({ extra: { sort_title: '{title}' } as ReturnType<typeof createNfoOptions>['extra'] })
    expect(old.extra.countries).toBe('')
    const grouped = applyNfoLayout(old, 'multipart')
    expect(grouped.extra.collection).toBe('{video_title}')
    expect(previewNfoTemplate(grouped.extra.sort_title)).toBe('示例视频 - 001')
    expect(previewNfoTemplate('{publish_date:%Y.%m.%d}-{index:04}')).toBe('2026.10.18-0002')
    expect(nfoOptionsError(grouped)).toBeNull()
    grouped.extra.studios = '{unknown}'
    expect(nfoOptionsError(grouped)).not.toBeNull()
    expect(old.extra.studios).toBe('')
    expect(applyNfoLayout(old, 'uploader').extra.tags).toBe('{uploader}, {genre}')
  })
  it('previews and validates human quality and date naming without changing raw quality', () => {
    const template = '{quality}_{quality_label}_{publish_date:%Y.%m.%d}'
    expect(validateNamingTemplate(template)).toBeNull()
    expect(previewTemplate(template, 'mp4')).toBe('80_1080P_2024.01.02.mp4')
    expect(validateNamingTemplate('{date:%Q}')).not.toBeNull()
  })
})
