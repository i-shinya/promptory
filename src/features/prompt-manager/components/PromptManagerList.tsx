import { PromptManager } from '../types'
import PromptManagerCard from './ui/PromptManagerCard'

export interface PromptManagerListProps {
  promptManagers: PromptManager[]
  onClickPromptManager: (id: number) => void
  onClickDeletePromptManager: (id: number) => void
}

const PromptManagerList = ({
  promptManagers,
  onClickPromptManager,
  onClickDeletePromptManager,
}: PromptManagerListProps) => {
  return (
    <div className="overflow-y-auto">
      {promptManagers.map((item) => (
        <PromptManagerCard
          key={item.id}
          item={item}
          onClickPromptManager={onClickPromptManager}
          onClickDeletePromptManager={onClickDeletePromptManager}
        />
      ))}
    </div>
  )
}

export default PromptManagerList
