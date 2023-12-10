import { invoke } from '@tauri-apps/api/tauri'

interface CreatePromptManagerRequest {
  title: string
}

export const createPromptManagerAction = async (
  title: string,
): Promise<number> => {
  const request: CreatePromptManagerRequest = { title }
  const response = await invoke('create_prompt_manager', { request })
  return Number(response)
}

interface LogicalPromptManagerRequest {
  id: number
}

interface LogicalPromptManagerResponse {
  id: number
}

export const logicalDeletePromptManagerAction = async (
  id: number,
): Promise<LogicalPromptManagerResponse> => {
  const request: LogicalPromptManagerRequest = { id }
  const response = (await invoke('logical_delete_prompt_manager', {
    request,
  })) as string
  return JSON.parse(response) as LogicalPromptManagerResponse
}
