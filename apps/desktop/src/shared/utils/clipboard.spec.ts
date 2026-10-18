import { invoke, isTauri } from '@tauri-apps/api/core'
import { beforeEach, describe, expect, it, vi } from 'vitest'

import { readClipboardText } from './clipboard'

vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
  isTauri: vi.fn(() => true),
}))

describe('readClipboardText', () => {
  beforeEach(() => {
    vi.mocked(invoke).mockReset()
  })

  it('reads Android clipboard through the native command instead of the WebView clipboard API', async () => {
    vi.mocked(invoke).mockResolvedValue('https://www.bilibili.com/video/BV1test')

    await expect(readClipboardText(true)).resolves.toBe('https://www.bilibili.com/video/BV1test')
    expect(invoke).toHaveBeenCalledWith('mobile_read_clipboard_text')
  })
})

it('reads the desktop native clipboard without browser permissions', async () => {
  vi.mocked(isTauri).mockReturnValue(true)
  vi.mocked(invoke).mockResolvedValue('BV1xx411c7mD')
  await expect(readClipboardText(false)).resolves.toBe('BV1xx411c7mD')
  expect(invoke).toHaveBeenCalledWith('desktop_read_clipboard_text')
})
