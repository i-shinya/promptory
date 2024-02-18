import IconButton from '@/components/ui/IconButton'
import { Progress } from '@/components/ui/progress'
import { Textarea } from '@/components/ui/textarea'

interface ComparingPromptRowProps {
  answer: string
  systemPrompt: string
  setSystemPrompt: (systemPrompt: string) => void
  deleteRow: () => void
  isLoading?: boolean
}

const ComparingResultRow: React.FC<ComparingPromptRowProps> = ({
  answer,
  systemPrompt,
  setSystemPrompt,
  deleteRow: removeRow,
  isLoading,
}) => {
  const progress = isLoading ? 30 : 100

  return (
    <>
      <div className="flex flex-row gap-2 grow">
        <div className="dark:bg-neutral-600 p-1 w-1/2">
          <Textarea
            value={systemPrompt}
            onChange={(e) => setSystemPrompt(e.target.value)}
            className="h-full"
            placeholder="Enter System Prompt."
          />
        </div>
        <div className="flex items-center">
          <span className="i-solar-double-alt-arrow-right-outline"></span>
        </div>
        {isLoading ? (
          <div className="p-1 w-1/2 flex items-center">
            <Progress className="dark:bg-neutral-600" value={progress} />
          </div>
        ) : (
          <div className="dark:bg-neutral-600 p-1 w-1/2 h-full">
            <div className="dark:bg-neutral-700 h-full p-1">{answer}</div>
          </div>
        )}
        <div className="flex items-center">
          <IconButton
            icon="i-solar-trash-bin-trash-bold"
            onClick={() => {
              removeRow()
            }}
          />
        </div>
      </div>
    </>
  )
}

export default ComparingResultRow
