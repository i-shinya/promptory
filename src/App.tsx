import { useState } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import './App.css'

interface ChatRequest {
  userPrompt: string
  systemPrompt: string
  model: string
  temperature: number
  responseFormat?: string
}

function App() {
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
      // TODO handler error
      console.error('Failed to post chat:', error)
    }
  }

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setRequest({
      ...request,
      [e.target.name]:
        e.target.name === 'temperature'
          ? Number(e.target.value)
          : e.target.value,
    })
  }

  return (
    <div className="container">
      <form
        onSubmit={(e) => {
          e.preventDefault()
          postChat()
        }}
      >
        <input
          className="row"
          name="userPrompt"
          value={request.userPrompt}
          onChange={handleChange}
          placeholder="Enter user prompt..."
        />
        <input
          className="row"
          name="systemPrompt"
          value={request.systemPrompt}
          onChange={handleChange}
          placeholder="Enter system prompt..."
        />
        <input
          className="row"
          name="model"
          value={request.model}
          onChange={handleChange}
          placeholder="Enter model..."
        />
        <input
          className="row"
          name="temperature"
          type="number"
          value={request.temperature}
          onChange={handleChange}
          placeholder="Enter temperature..."
          min="0"
          max="1"
          step="0.1"
        />
        <button type="submit">Submit</button>
      </form>

      <p>{answer}</p>
    </div>
  )
}

export default App
