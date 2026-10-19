import { computed, shallowRef } from 'vue'
import { commandDiagnostic } from './commandDiagnostic'

const entries = shallowRef<string[]>([])
export const recentFailureReport = computed(() => entries.value.join('\n\n---\n\n'))
export const clearRecentFailures = () => { entries.value = [] }

// Keep a bounded, redacted session history without request arguments (cookies,
// QR keys, and complete settings must never enter the diagnostic report).
export const recordCommandFailure = (command: string, error: unknown) => {
  entries.value = [commandDiagnostic(error, `操作: ${command}`).slice(0, 65536), ...entries.value].slice(0, 20)
}
