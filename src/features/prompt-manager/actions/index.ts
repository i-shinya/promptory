import { invoke } from '@tauri-apps/api/tauri'
import {
  ActionType,
  ApiType,
  PromptManager,
} from '@/features/prompt-manager/types'

interface GetPromptManagerRequest {
  id: number
}

type GetPromptManagerResponse = PromptManager

export const getPromptManagerAction = async (
  id: number,
): Promise<GetPromptManagerResponse> => {
  const request: GetPromptManagerRequest = { id }
  const response = (await invoke('get_prompt_manager', {
    request,
  })) as string
  return JSON.parse(response) as GetPromptManagerResponse
}

interface GetPromptManagersRequest {}

interface GetPromptManagersResponse {
  managers: PromptManager[]
}

export const getAllPromptManagersAction = async (): Promise<
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

interface UpdatePromptManagerRequest {
  id: number
  title: string
  actionType?: ActionType
  apiType?: ApiType
}

export const updatePromptManagerAction = async (
  id: number,
  title: string,
  actionType?: ActionType,
  apiType?: ApiType,
): Promise<void> => {
  const request: UpdatePromptManagerRequest = { id, title, actionType, apiType }
  await invoke('update_prompt_manager', {
    request,
  })
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
