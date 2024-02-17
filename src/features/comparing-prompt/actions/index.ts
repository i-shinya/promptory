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

export interface AddPromptRequest {
  managerId: number
}

interface AddPromptResponse {
  id: number
}

export const addComparingPromptSettingAction = async (
  request: AddPromptRequest,
): Promise<AddPromptResponse> => {
  const response = (await invoke('add_comparing_prompt_setting', {
    request,
  })) as string
  return JSON.parse(response) as AddPromptResponse
}
