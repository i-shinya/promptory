import { useParams } from 'react-router-dom'
import { Separator } from '@/components/ui/separator'
import PromptManagerEditForm from '@/features/prompt-manager/components/PromptManagerEditForm'
import ComparingPrompt from '@/features/comparing-prompt/components/ComparingPrompt'
import { useState } from 'react'

const PromptManager = () => {
  const { id } = useParams()

  const [canExecutePrompts, setCanExecutePrompts] = useState(false)

  return (
    <div className="flex flex-col h-full max-h-full">
      {id ? (
        <>
          <PromptManagerEditForm
            id={id}
            setCanExecutePrompts={setCanExecutePrompts}
          />
          <Separator className="my-3" />
          <ComparingPrompt
            managerId={id}
            canExecutePrompts={canExecutePrompts}
          />
        </>
      ) : (
        <div>Invalid ID provided.</div>
      )}
    </div>
  )
}

export default PromptManager
