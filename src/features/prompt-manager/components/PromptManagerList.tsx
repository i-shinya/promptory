import Card from '../../../components/ui/Card'
import { PromptManager } from '../types'
import TemporaryPromptManagerCard from './ui/TemporaryPromptManagerCard'
import PromptManagerCard from './ui/PromptManagerCard'

export interface PromptManagerListProps {
  promptManagers: PromptManager[]
  onClickPromptManager: (id: number) => void
  onClickDeletePromptManager: (id: number) => void
  handleSavePromptManager: (title: string) => void
  isVisibleNewManagerForm: boolean
  setIsVisibleNewManagerForm: (isVisible: boolean) => void
}

const PromptManagerList = ({
  promptManagers,
  onClickPromptManager,
  onClickDeletePromptManager,
  handleSavePromptManager: handleSavePromptManager,
  isVisibleNewManagerForm: isTemporaryPromptManagerVisible,
  setIsVisibleNewManagerForm: setIsTemporaryPromptManagerVisible,
}: PromptManagerListProps) => {
  return (
    <div className="overflow-y-auto">
      {promptManagers.length === 0 ? (
        <Card>
          <span className="font-bold">No prompts</span>
        </Card>
      ) : (
        <>
          {promptManagers.map((item, index) => (
            <PromptManagerCard
              key={index}
              item={item}
              onClickPromptManager={onClickPromptManager}
              onClickDeletePromptManager={onClickDeletePromptManager}
            />
          ))}
          {isTemporaryPromptManagerVisible && (
            <TemporaryPromptManagerCard
              handleSubmit={handleSavePromptManager}
              onClickCancel={() => setIsTemporaryPromptManagerVisible(false)}
            />
          )}
        </>
      )}
    </div>
  )
}

export default PromptManagerList
