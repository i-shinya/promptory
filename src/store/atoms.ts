import { PromptManager } from '@/features/prompt-manager/types'
import { atom } from 'recoil'

export enum PromptManagersKey {
  PROMPT_MANAGERS = 'promptManagers',
}

export const promptManagersAtom = atom<PromptManager[]>({
  key: PromptManagersKey.PROMPT_MANAGERS,
  default: [],
})
