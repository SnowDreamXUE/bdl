import type { NfoExtraFields, NfoOptions } from '../api/dto'

export const createNfoExtraFields = (): NfoExtraFields => ({
  sort_title: '', original_title: '', tagline: '', outline: '', genres: '', tags: '',
  collection: '', collection_overview: '', directors: '', writers: '', studios: '', countries: '',
})
export const createNfoOptions = (): NfoOptions => ({
  title_template: '{title}', plot_template: '{description}\nUP 主：{uploader}\n{source_url}',
  include_plot: true, include_date: true, include_runtime: true, include_genre: true,
  include_source: true, include_cover: true, uploader_as_actor: false, custom_tags: '', extra: createNfoExtraFields(),
})
export const normalizeNfoOptions = (options?: Partial<NfoOptions>): NfoOptions => ({
  ...createNfoOptions(), ...options, extra: { ...createNfoExtraFields(), ...options?.extra },
})

export const nfoVariables = [
  { name: 'title', desc: '任务标题（默认展示名称）', example: '示例视频 - 开场' },
  { name: 'video_title', desc: '视频总标题（多 P 共用）', example: '示例视频' },
  { name: 'part_title', desc: '分 P 标题', example: '开场' },
  { name: 'part_index', desc: '分 P 序号，从 1 开始', example: '1' },
  { name: 'part_index:03', desc: '分 P 序号，补齐三位', example: '001' },
  { name: 'index', desc: '来源列表中的视频序号', example: '2' },
  { name: 'index:03', desc: '列表序号，补齐三位', example: '002' },
  { name: 'source_title', desc: '本次解析的来源标题', example: '示例收藏夹' },
  { name: 'description', desc: '平台提供的视频简介', example: '视频简介示例' },
  { name: 'uploader', desc: 'UP 主名称', example: '示例UP' },
  { name: 'owner_mid', desc: 'UP 主 UID', example: '1001' },
  { name: 'genre', desc: '平台分区名称', example: '知识' },
  { name: 'publish_date', desc: '发布日期，年-月-日', example: '2026-10-18' },
  { name: 'publish_date:%Y.%m.%d', desc: '发布日期，年.月.日', example: '2026.10.18' },
  { name: 'date', desc: '任务创建日期，年-月-日', example: '2026-10-19' },
  { name: 'date:%Y%m%d', desc: '任务创建日期，紧凑格式', example: '20261019' },
  { name: 'bvid', desc: 'BV 号', example: 'BV1xx411c7mD' },
  { name: 'aid', desc: 'AV 数字编号', example: '170001' },
  { name: 'cid', desc: '分 P 内容编号', example: '9001' },
  { name: 'source_url', desc: '原视频或课程链接（含分 P）', example: 'https://www.bilibili.com/video/BV1xx411c7mD?p=1' },
  { name: 'quality_label', desc: '所选视频清晰度名称', example: '1080P' },
  { name: 'quality', desc: '所选视频清晰度编号', example: '80' },
  { name: 'codec', desc: '所选视频编码', example: 'avc' },
  { name: 'duration_seconds', desc: '视频时长，单位秒', example: '180' },
] as const
export const nfoExamples: Record<string, string> = Object.fromEntries(nfoVariables.map(v => [v.name, v.example]))
nfoExamples.owner_name = nfoExamples.uploader
nfoExamples.collection_title = nfoExamples.source_title

export const nfoExtraDefinitions: { key: keyof NfoExtraFields; label: string; helper: string }[] = [
  { key: 'sort_title', label: '排序标题', helper: '只影响媒体库排序，不改变显示标题。例如：{uploader} - {publish_date} - {part_index:03}。' },
  { key: 'original_title', label: '原始标题', helper: '保留原名称，例如 {video_title}。留空不写入。' },
  { key: 'tagline', label: '一句话介绍', helper: '标题旁的简短介绍，显示位置由播放器决定。' },
  { key: 'outline', label: '短简介', helper: '供简略资料视图显示，与完整简介分开。' },
  { key: 'genres', label: '附加分类', helper: '例如：教程, 科技。关闭自动分区后，可完全使用这里的分类。' },
  { key: 'tags', label: '动态标签', helper: '例如：{uploader}, {quality_label}。与上方固定标签合并，逗号分隔。' },
  { key: 'collection', label: '作品集名称', helper: '将独立视频归到同一作品集，例如 {uploader} 或 {video_title}。不是电视剧的季/集编号。' },
  { key: 'collection_overview', label: '作品集简介', helper: '需要同时填写作品集名称，部分播放器才显示。' },
  { key: 'directors', label: '导演', helper: '仅填写实际导演；不会自动将 UP 主当作导演。多位用逗号分隔。' },
  { key: 'writers', label: '编剧', helper: '仅填写实际编剧，多位用逗号分隔。' },
  { key: 'studios', label: '制作机构', helper: '仅填写实际机构，不根据 UP 主或平台推断。多项用逗号分隔。' },
  { key: 'countries', label: '制作国家或地区', helper: '仅填写已知信息，多项用逗号分隔。' },
]

export const nfoTemplateError = (template: string): string | null => {
  const remaining = template.replace(/\{([^{}]+)\}/g, (whole, key: string) => {
    const name = key.trim()
    return Object.hasOwn(nfoExamples, name) || /^(date|publish_date):%Y(?:-%m-%d|\.%m\.%d|%m%d)$/.test(name)
      || /^(part_index|index):0[234]$/.test(name) ? '' : whole
  })
  return /[{}]/.test(remaining) ? '模板变量未知或未闭合，请从插入菜单选择' : null
}
export const previewNfoTemplate = (template: string): string => template.replace(/\{([^{}]+)\}/g, (_, key: string) => {
  const [name, format] = key.trim().split(':')
  const raw = nfoExamples[name] ?? ''
  if (!format) return raw
  if (['date', 'publish_date'].includes(name)) {
    if (format === '%Y.%m.%d') return raw.replaceAll('-', '.')
    if (format === '%Y%m%d') return raw.replaceAll('-', '')
    return format === '%Y-%m-%d' ? raw : ''
  }
  return /^(02|03|04)$/.test(format) ? raw.padStart(Number(format), '0') : ''
})
export const nfoOptionsError = (options: NfoOptions): string | null => {
  if (!options.title_template.trim()) return 'NFO 标题模板不能为空'
  for (const template of [options.title_template, ...(options.include_plot ? [options.plot_template] : []), ...Object.values(options.extra ?? {})]) {
    if (nfoTemplateError(template)) return 'NFO 模板变量未知或未闭合'
  }
  return null
}

export const applyNfoLayout = (options: NfoOptions, layout: string): NfoOptions => {
  const result = normalizeNfoOptions(options)
  result.extra = createNfoExtraFields()
  result.title_template = '{title}'
  result.plot_template = createNfoOptions().plot_template
  if (layout === 'uploader') {
    result.extra.collection = '{uploader}'
    result.extra.sort_title = '{uploader} - {publish_date} - {video_title} - {part_index:03}'
    result.extra.tags = '{uploader}, {genre}'
  } else if (layout === 'multipart') {
    result.title_template = '{video_title} - P{part_index} - {part_title}'
    result.extra.collection = '{video_title}'
    result.extra.sort_title = '{video_title} - {part_index:03}'
    result.extra.original_title = '{part_title}'
  }
  return result
}
