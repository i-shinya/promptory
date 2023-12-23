import { Textarea } from '@/components/ui/textarea'

interface ComparingPromptRowProps {
  answer: string
  systemPrompt: string
  setSystemPrompt: (systemPrompt: string) => void
}

const ComparingResultRow: React.FC<ComparingPromptRowProps> = ({
  answer,
  systemPrompt,
  setSystemPrompt,
}) => {
  return (
    <div className="flex flex-row gap-2 grow">
      <div className="dark:bg-neutral-600 p-1 h-full w-full">
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
      <div className="dark:bg-neutral-600 p-1 h-full w-full">
        <div className="dark:bg-neutral-700 h-full p-1">{answer}</div>
      </div>
    </div>
  )
}

export default ComparingResultRow
