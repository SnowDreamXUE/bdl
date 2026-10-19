import { redactDiagnosticMessage } from '../stores/transferView'

export const commandDiagnostic = (error: unknown, context: string): string => {
  const value = error && typeof error === 'object' ? error as Record<string, unknown> : {}
  return redactDiagnosticMessage([
    `时间: ${new Date().toISOString()}`,
    context,
    typeof value.code === 'string' ? `错误码: ${value.code}` : '',
    typeof value.message === 'string' ? value.message : String(error),
    typeof value.detail === 'string' ? value.detail : '',
  ].filter(Boolean).join('\n'))
}
