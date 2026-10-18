import { invoke, isTauri } from '@tauri-apps/api/core'

import { isAndroidPlatform } from '../platform/environment'

export const readClipboardText = async (android = isAndroidPlatform()): Promise<string> => {
  if (android) return invoke<string>('mobile_read_clipboard_text')
  if (isTauri()) return invoke<string>('desktop_read_clipboard_text')
  return navigator.clipboard.readText()
}
