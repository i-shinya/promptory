import { useState } from 'react'
import RunSettingEditForm from './part/RunSettingEditForm'
import { ComparingPromtpRow, RunSettings } from '../types'
import { toast } from 'react-toastify'
import { RunChatRequest, runChatAction } from '../actions'
import { Separator } from '@/components/ui/separator'
import IconButton from '@/components/ui/IconButton'
import ComparingRowList from './part/ComparingRowList'

interface PromptManagerEditFormProps {
  managerId: string
}

const ComparingPrompt: React.FC<PromptManagerEditFormProps> = ({
  managerId,
}) => {
  const [comparingRows, setComparingRows] = useState<ComparingPromtpRow[]>([])

  // idが一致するrowのanswerを更新する
  const setAnswer = (id: number, answer: string) => {
    const newSystemPrompts = [...comparingRows]
    const index = newSystemPrompts.findIndex((item) => item.id === id)
    newSystemPrompts[index].answer = answer
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

  const addSystemPrompt = () => {
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
