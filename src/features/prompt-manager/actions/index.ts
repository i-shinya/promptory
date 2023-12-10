import { invoke } from '@tauri-apps/api/tauri'

interface CreatePromptManagerRequest {
  title: string
}

interface CreatePromptManagerResponse {
  id: number
}

export const createPromptManagerAction = async (
  title: string,
): Promise<CreatePromptManagerResponse> => {
  const request: CreatePromptManagerRequest = { title }
  const response = (await invoke('create_prompt_manager', {
    request,
  })) as string
  return JSON.parse(response) as CreatePromptManagerResponse
}

interface LogicalPromptManagerRequest {
  id: number
}

export const logicalDeletePromptManagerAction = async (
  id: number,
): Promise<void> => {
  const request: LogicalPromptManagerRequest = { id }
  await invoke('logical_delete_prompt_manager', {
    request,
  })
}
