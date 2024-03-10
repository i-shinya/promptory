import { invoke } from '@tauri-apps/api/tauri'

export interface RunChatRequest {
  runId: number
  userPrompt: string
  systemPrompt: string
  providerType: string
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

export interface GetComparingPromptSettingRequest {
  managerId: number
}

interface GetComparingPromptSettingResponse {
  settings: ComparingPromptSetting[]
}

interface ComparingPromptSetting {
  id: number
  managerId: number
  version: number
  systemPrompt: string
}

export const getComparingPromptSettingAction = async (
  request: GetComparingPromptSettingRequest,
): Promise<GetComparingPromptSettingResponse> => {
  const response = (await invoke('get_all_comparing_prompt_settings', {
    request,
  })) as string
  return JSON.parse(response) as GetComparingPromptSettingResponse
}

export interface SaveComparingPromptRunRequest {
  managerId: number
  userPrompt: string
  providerType: string
  model: string
  temperature: number
  maxToken?: number
  responseFormat?: string
}

interface SaveComparingPromptRunResponse {
  id: number
}

export const saveComparingPromptRunAction = async (
  request: SaveComparingPromptRunRequest,
): Promise<SaveComparingPromptRunResponse> => {
  const response = (await invoke('save_comparing_prompt_run', {
    request,
  })) as string
  return JSON.parse(response) as SaveComparingPromptRunResponse
}
