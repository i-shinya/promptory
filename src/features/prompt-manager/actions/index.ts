import { invoke } from '@tauri-apps/api/tauri'
import { PromptManager } from '@/features/prompt-manager/types'

interface GetPromptManagersRequest {}

interface GetPromptManagersResponse {
  managers: PromptManager[]
}

export const getPromptManagersAction = async (): Promise<
  GetPromptManagersResponse
> => {
  const request: GetPromptManagersRequest = {}
  const response = (await invoke('get_all_prompt_managers', {
    request,
  })) as string
  return JSON.parse(response) as GetPromptManagersResponse
}

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
