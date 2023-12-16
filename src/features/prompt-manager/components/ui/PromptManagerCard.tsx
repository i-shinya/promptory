import { Badge } from '@/components/ui/badge'
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
          <span className="px-2 text-sm border-l-4 border-solid dark:border-white">
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
            <Badge className="dark:bg-gray-600 dark:text-white">{tag}</Badge>
          ))}
        </div>
      </div>
    </Card>
  )
}

export default PromptManagerCard
