import { useParams } from 'react-router-dom'
import { Separator } from '@/components/ui/separator'
import PromptManagerEditForm from '@/features/prompt-manager/components/PromptManagerEditForm'
import ComparingPrompt from '@/features/comparing-prompt/components/ComparingPrompt'

const PromptManager = () => {
  const { id } = useParams()

  return (
    <div className="flex flex-col h-full max-h-full">
      {id ? (
        <>
          <PromptManagerEditForm id={id} />
          <Separator className="my-3" />
          <ComparingPrompt managerId={id} />
        </>
      ) : (
        <div>Invalid ID provided.</div>
      )}
    </div>
  )
}

export default PromptManager
