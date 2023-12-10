import { FormEvent, useState } from 'react'
import Card from '../../../../components/ui/Card'
import ButtonWithIcon from '../../../../components/ui/ButtonWithIcon'

export interface TemporaryPromptManagerCardProps {
  handleSubmit: (title: string) => void
  onClickCancel: () => void
}

const TemporaryPromptManagerCard = ({
  handleSubmit,
  onClickCancel,
}: TemporaryPromptManagerCardProps) => {
  const [title, setTitle] = useState('')

  return (
    <Card>
      <form
        onSubmit={(e: FormEvent) => {
          const value = title.trim()
          e.preventDefault()
          if (value === '') {
            return
          }
          handleSubmit(value)
        }}
      >
        <div className="flex flex-col gap-px cursor-pointer">
          <div className="dark:bg-neutral-600 p-1">
            <input
              className="text-md dark:bg-neutral-700 outline-none w-full align-middle"
              value={title}
              onChange={(e) => setTitle(e.target.value)}
              required
            />
          </div>
          <div className="flex justify-end gap-2 mt-2">
            <ButtonWithIcon
              text="Cancel"
              type="button"
              icon="i-solar-close-circle-bold"
              color="warn"
              onClick={onClickCancel}
            />
            <ButtonWithIcon
              text="Save"
              type="submit"
              icon="i-solar-add-folder-bold"
              color="info"
            />
          </div>
        </div>
      </form>
    </Card>
  )
}

export default TemporaryPromptManagerCard
