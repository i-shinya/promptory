export type ActionType = 'ComparingPrompt' | 'ComparingModel'
export type ApiType = 'Chat' | 'Vision'

export interface PromptManager {
  id: number
  title: string
  actionType: ActionType | null
  apiType: ApiType | null
  tags: string[]
}
