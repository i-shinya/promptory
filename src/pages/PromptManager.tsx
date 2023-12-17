import { useState } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import { PromptManager as PromptManagerType } from '@/features/prompt-manager/types'
import { useParams } from 'react-router-dom'
import { Separator } from '@/components/ui/separator'
import PromptManagerEditForm from '@/features/prompt-manager/components/PromptManagerEditForm'

interface ChatRequest {
  userPrompt: string
  systemPrompt: string
  model: string
  temperature: number
  responseFormat?: string
}

const PromptManager = () => {
  const { id } = useParams()

  const [answer, setAnswer] = useState('')
  const [request, setRequest] = useState<ChatRequest>({
    userPrompt: 'enter user prompt...',
    systemPrompt: 'enter system prompt...',
    model: 'gpt-4-1106-preview',
    temperature: 0,
  })

  const postChat = async () => {
    try {
      const response = await invoke('post_chat', { request })
      setAnswer(response as string)
    } catch (error) {
      // TODO react notificationを追加してエラー時に表示する
      console.error('Failed to post chat:', error)
    }
  }

  return (
    <div>
      {id ? <PromptManagerEditForm id={id} /> : <div>Invalid ID provided.</div>}
      <Separator className="my-4" />
    </div>
  )
}

export default PromptManager
