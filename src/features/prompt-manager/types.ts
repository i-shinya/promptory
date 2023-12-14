export type ActionType = 'ComparingPrompt' | 'ComparingModel'

export interface PromptManager {
  id: number
  title: string
  actionType: ActionType | null
  tags: string[]
}
