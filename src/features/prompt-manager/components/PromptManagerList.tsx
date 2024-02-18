import { useEffect } from 'react'
import Card from '../../../components/ui/Card'
import { PromptManager } from '../types'
import NewPromptManagerInputForm from './ui/NewPromptManagerInputForm'
import PromptManagerCard from './ui/PromptManagerCard'
import {
  createPromptManagerAction,
  getAllPromptManagersAction,
  logicalDeletePromptManagerAction,
} from '../actions'
import { useNavigate } from 'react-router-dom'
import { toast } from 'react-toastify'
import { useRecoilValue, useSetRecoilState } from 'recoil'
import { promptManagersAtom } from '@/store/atoms'

export interface PromptManagerListProps {
  isVisibleNewManagerForm: boolean
  setIsVisibleNewManagerForm: (isVisible: boolean) => void
}

const PromptManagerList = ({
  isVisibleNewManagerForm: isTemporaryPromptManagerVisible,
  setIsVisibleNewManagerForm,
}: PromptManagerListProps) => {
  const navigate = useNavigate()

  const promptManagers = useRecoilValue<PromptManager[]>(promptManagersAtom)
  const setPromptManagers = useSetRecoilState<PromptManager[]>(
    promptManagersAtom,
  )

  useEffect(() => {
    const fetchPromptManagers = async () => {
      try {
        const res = await getAllPromptManagersAction()
        console.log(res)
        setPromptManagers(res.managers)
      } catch (error) {
        toast.error(`Failed to fetch prompt manager: ${error}`)
      }
    }
    fetchPromptManagers()
  }, [])

  const selectPromptManager = (id: number) => {
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
      selectPromptManager(id)
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
    <>
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
    </>
  )
}

export default PromptManagerList
