export type ApiType = 'Chat' | 'Assistant'

export interface PromptManager {
  id: number
  title: string
  apiType: ApiType | null
  tags: string[]
}
