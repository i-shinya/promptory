import { useEffect, useState } from 'react'
import Card from '../../../components/ui/Card'
import { PromptManager } from '../types'
import NewPromptManagerInputForm from './ui/NewPromptManagerInputForm'
import PromptManagerCard from './ui/PromptManagerCard'
import {
  createPromptManagerAction,
  getPromptManagersAction,
  logicalDeletePromptManagerAction,
} from '../actions'
import { useNavigate } from 'react-router-dom'
import { toast } from 'react-toastify'

export interface PromptManagerListProps {
  isVisibleNewManagerForm: boolean
  setIsVisibleNewManagerForm: (isVisible: boolean) => void
}

const PromptManagerList = ({
  isVisibleNewManagerForm: isTemporaryPromptManagerVisible,
  setIsVisibleNewManagerForm,
}: PromptManagerListProps) => {
  const navigate = useNavigate()

  const [promptManagers, setPromptManagers] = useState<PromptManager[]>([])

  useEffect(() => {
    const fetchPromptManagers = async () => {
      try {
        const res = await getPromptManagersAction()
        console.log(res)
        setPromptManagers(res.managers)
      } catch (error) {
        toast.error(`Failed to fetch prompt manager: ${error}`)
      }
    }
    fetchPromptManagers()
  }, [])

  const selectPromptManager = (id: number) => {
    // TODO 詳細画面を作成したらそちらに遷移するようにする
    console.log(`select ${id}`)
    const URL_PATH = '/prompt_manager/:id'
    navigate(URL_PATH.replace(':id', id.toString()))
  }

  const savePromptManager = async (title: string) => {
    try {
      const res = await createPromptManagerAction(title)
      const id = res.id
      console.log(res.id)
      // 状態の更新
      setIsVisibleNewManagerForm(false)
      setPromptManagers([
        ...promptManagers,
        { id, title, actionType: null, apiType: null, tags: [] },
      ])
      toast.info('Save Prompt Manager Success!')
    } catch (error) {
      toast.error(`Failed to create prompt manager: ${error}`)
    }
  }

  const deletePromptManager = async (id: number) => {
    try {
      await logicalDeletePromptManagerAction(id)
      removePromptManager(id)
    } catch (error) {
      toast.error(`Failed to logical delete prompt manager: ${error}`)
    }
  }

  const removePromptManager = (id: number) => {
    setPromptManagers(promptManagers.filter((item) => item.id !== id))
  }

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
              onClickPromptManager={selectPromptManager}
              onClickDeletePromptManager={deletePromptManager}
            />
          ))}
        </>
      )}
      {isTemporaryPromptManagerVisible && (
        <NewPromptManagerInputForm
          handleSubmit={savePromptManager}
          onClickCancel={() => setIsVisibleNewManagerForm(false)}
        />
      )}
    </div>
  )
}

export default PromptManagerList
