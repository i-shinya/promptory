import Card from '../../../../components/ui/Card'
import IconButton from '../../../../components/ui/IconButton'
import { PromptManager } from '../../types'

export interface PromptManagerCardProps {
  item: PromptManager
  onClickPromptManager: (id: number) => void
  onClickDeletePromptManager: (id: number) => void
}

const PromptManagerCard = ({
  item,
  onClickPromptManager,
  onClickDeletePromptManager,
}: PromptManagerCardProps) => {
  return (
    <Card>
      <div
        className="flex flex-col gap-px cursor-pointer"
        onClick={(e) => {
          e.stopPropagation()
          onClickPromptManager(item.id)
        }}
      >
        <div className="flex flex-row justify-between">
          <span className="px-2 text-sm border-l-4 border-solid border-white">
            {item.actionType}
          </span>
          <IconButton
            icon="i-solar-trash-bin-trash-bold"
            onClick={() => {
              onClickDeletePromptManager(item.id)
            }}
          />
        </div>
        <div className="text-lg">
          <span>{item.title}</span>
        </div>
        <div className="flex gap-px">
          {item.tags.map((tag) => (
            <span
              key={tag}
              className="px-2 text-xs border border-solid rounded-2xl border-gray-400"
            >
              {tag}
            </span>
          ))}
        </div>
      </div>
    </Card>
  )
}

export default PromptManagerCard
