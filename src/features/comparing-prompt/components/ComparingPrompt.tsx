import { useState } from 'react'
import ComparingResultRow from './ui/ComparingResltRow'
import RunSettingEditForm from './part/RunSettingEditForm'
import { ComparingPromtpRow, RunSettings } from '../types'
import { toast } from 'react-toastify'
import { RunChatRequest, runChatAction } from '../actions'

interface PromptManagerEditFormProps {
  managerId: string
}

const ComparingPrompt: React.FC<PromptManagerEditFormProps> = ({
  managerId,
}) => {
  const [comparingRows, setComparingRows] = useState<ComparingPromtpRow[]>([
    {
      id: 1,
      systemPrompt: '',
      answer: '',
    },
  ])

  // idが一致するrowのanswerを更新する
  const setAnswer = (id: number, answer: string) => {
    const newSystemPrompts = [...comparingRows]
    const index = newSystemPrompts.findIndex((item) => item.id === id)
    newSystemPrompts[index].answer = answer
    setComparingRows(newSystemPrompts)
  }

  // idが一致するrowのsystemPromptを更新する
  const setSytemPrompt = (id: number, systemPrompt: string) => {
    const newSystemPrompts = [...comparingRows]
    const index = newSystemPrompts.findIndex((item) => item.id === id)
    newSystemPrompts[index].systemPrompt = systemPrompt
    setComparingRows(newSystemPrompts)
  }

  const run = async (setting: RunSettings) => {
    // runを登録してrunIdを取得する
    const runId = 1
    await Promise.all(
      comparingRows.map(async (item) => {
        try {
          const request: RunChatRequest = {
            runId: runId,
            systemPrompt: item.systemPrompt,
            ...setting,
          }
          const response = await runChatAction(request)
          setAnswer(item.id, response.answer)
        } catch (error) {
          toast.error(`Failed to update prompt manager: ${error}`)
        }
      }),
    )
  }

  return (
    <div className="flex row h-full gap-4">
      {/* この部分をコンポーネント化する */}
      <div className="flex flex-col gap-4 grow overflow-y-auto w-2/5 h-full">
        {comparingRows.map((item) => (
          <ComparingResultRow
            key={item.id}
            answer={item.answer}
            systemPrompt={item.systemPrompt}
            setSystemPrompt={(systemPrompt) => {
              setSytemPrompt(item.id, systemPrompt)
            }}
          />
        ))}
      </div>
      <div className="w-2/5 h-full">
        <RunSettingEditForm managerId={managerId} runAction={run} />
      </div>
    </div>
  )
}

export default ComparingPrompt
