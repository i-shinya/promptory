import { invoke } from '@tauri-apps/api/tauri'

export interface RunChatRequest {
  runId: number
  userPrompt: string
  systemPrompt: string
  model: string
  temperature: number
  maxToken?: number
  responseFormat?: string
}

interface RunChatResponse {
  answer: string
}

export const runChatAction = async (
  request: RunChatRequest,
): Promise<RunChatResponse> => {
  const response = (await invoke('run_comparing_prompt', {
    request,
  })) as string
  return JSON.parse(response) as RunChatResponse
}
