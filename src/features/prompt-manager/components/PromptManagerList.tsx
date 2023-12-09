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
      {promptManagers.length === 0 ? (
        <div className="dark:bg-zinc-800 p-2 m-2 rounded shadow cursor-pointer">
          <span className="font-bold">No prompts</span>
        </div>
      ) : (
        <>
          {promptManagers.map((item) => (
            <PromptManagerCard
              key={item.id}
              item={item}
              onClickPromptManager={onClickPromptManager}
              onClickDeletePromptManager={onClickDeletePromptManager}
            />
          ))}
        </>
      )}
    </div>
  )
}

export default PromptManagerList
