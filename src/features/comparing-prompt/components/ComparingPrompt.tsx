import { useState } from 'react'
import RunSettingEditForm from './part/RunSettingEditForm'
import { ComparingPromtpRow, RunSettings } from '../types'
import { toast } from 'react-toastify'
import {
  RunChatRequest,
  SaveComparingPromptRunRequest,
  runChatAction,
  saveComparingPromptRunAction,
} from '../actions'
import { Separator } from '@/components/ui/separator'
import IconButton from '@/components/ui/IconButton'
import ComparingRowList from './part/ComparingRowList'

interface PromptManagerEditFormProps {
  managerId: string
  canExecutePrompts: boolean
}

const ComparingPrompt: React.FC<PromptManagerEditFormProps> = ({
  managerId,
  canExecutePrompts,
}) => {
  const [comparingRows, setComparingRows] = useState<ComparingPromtpRow[]>([])
  const [isLoading, setIsLoading] = useState(false)

  // idが一致するrowのanswerを更新する
  const setAnswer = (id: number, answer: string) => {
    const newSystemPrompts = [...comparingRows]
    const index = newSystemPrompts.findIndex((item) => item.id === id)
    if (index === -1) {
      return
    }
    newSystemPrompts[index].answer = answer
    setComparingRows(newSystemPrompts)
  }

  const run = async (setting: RunSettings) => {
    if (!canExecutePrompts) {
      toast.warn('Please select and save ActionType.')
      return
    }
    if (comparingRows.length === 0) {
      toast.warn('Please add System Prompt.')
      return
    }

    setIsLoading(true)
    const saveRequest: SaveComparingPromptRunRequest = {
      managerId: Number(managerId),
      userPrompt: setting.userPrompt,
      providerType: 'OpenAI',
      model: setting.model,
      temperature: setting.temperature,
    }
    let runId = 0
    try {
      const saveResponse = await saveComparingPromptRunAction(saveRequest)
      runId = saveResponse.id
    } catch (error) {
      setIsLoading(false)
      toast.error(`Failed to saveComparingPromptRunAction: ${error}`)
      return
    }

    await Promise.all(
      comparingRows.map(async (item) => {
        try {
          const request: RunChatRequest = {
            runId: runId,
            systemPrompt: item.systemPrompt,
            providerType: 'OpenAI',
            ...setting,
          }
          const response = await runChatAction(request)
          setAnswer(item.id, response.answer)
        } catch (error) {
          toast.error(`Failed to runChatAction: ${error}`)
        }
      }),
    ).finally(() => {
      setIsLoading(false)
    })
  }

  const addSystemPrompt = () => {
    if (!canExecutePrompts) {
      toast.warn('Please select and save ActionType.')
      return
    }

    const newSystemPrompts = [...comparingRows]
    const id =
      newSystemPrompts.length > 0
        ? Math.max(...newSystemPrompts.map((item) => item.id)) + 1
        : 1
    newSystemPrompts.push({
      id: id,
      systemPrompt: '',
      answer: '',
    })
    setComparingRows(newSystemPrompts)
  }

  if (!canExecutePrompts) {
    return <div>Please select and save ActionType.</div>
  }

  return (
    <div
      className="flex flex-row gap-4"
      // TODO ここの値は適当なのでそのうち調整する
      style={{
        height: 'calc(100vh - 9rem)',
        maxHeight: 'calc(100vh - 9rem)',
      }}
    >
      <div className="flex flex-col grow w-2/5">
        <div className="max-h-12">
          <div className="flex flex-row">
            <IconButton
              className="mr-1"
              icon="i-solar-add-circle-linear"
              onClick={addSystemPrompt}
            />
            <span className="cursor-pointer" onClick={addSystemPrompt}>
              Add Row
            </span>
          </div>
          <Separator className="my-3" />
        </div>
        <div className="flex flex-col overflow-y-scroll">
          <ComparingRowList
            comparingRows={comparingRows}
            setComparingRows={setComparingRows}
            isLoading={isLoading}
          />
        </div>
      </div>
      <div className="flex flex-col w-1/3 overflow-y-scroll">
        <RunSettingEditForm managerId={managerId} runAction={run} />
      </div>
    </div>
  )
}

export default ComparingPrompt
