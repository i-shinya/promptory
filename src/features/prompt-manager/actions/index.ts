import { invoke } from '@tauri-apps/api/tauri'

interface CreatePromptManagerRequest {
  title: string
}

export const createPromptManagerAction = async (
  title: string,
): Promise<number> => {
  const request: CreatePromptManagerRequest = { title }
  const response = await invoke('create_prompt_manager', { request })
  return response as number
}
