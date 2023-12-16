import { useEffect, useState } from 'react'
import PromptManagerList from '../../features/prompt-manager/components/PromptManagerList'
import { PromptManager } from '../../features/prompt-manager/types'
import SideMenuHeader from './SideMenuHeader'
import { useNavigate } from 'react-router-dom'
import {
  createPromptManagerAction,
  getPromptManagersAction,
  logicalDeletePromptManagerAction,
} from '../../features/prompt-manager/actions'

const SideMenu = () => {
  const navigate = useNavigate()

  const [isVisibleNewManagerForm, setIsVisibleNewManagerForm] = useState(false)
  const [promptManagers, setPromptManagers] = useState<PromptManager[]>([])

  useEffect(() => {
    const fetchPromptManagers = async () => {
      try {
        const res = await getPromptManagersAction()
        console.log(res)
        setPromptManagers(res.managers)
      } catch (error) {
        // TODO react notificationを追加してエラー時にトーストを表示するようにする
        console.error('Failed to fetch prompt managers:', error)
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

  const goToConfigPage = () => {
    navigate('/config')
  }

  const addTemporaryPromptManager = () => {
    setIsVisibleNewManagerForm(true)
  }

  const savePromptManager = async (title: string) => {
    try {
      const res = await createPromptManagerAction(title)
      const id = res.id
      console.log(res.id)
      setIsVisibleNewManagerForm(false)
      setPromptManagers([
        ...promptManagers,
        { id, title, actionType: null, apiType: null, tags: [] },
      ])
    } catch (error) {
      // TODO react notificationを追加してエラー時にトーストを表示するようにする
      console.error('Failed to create prompt manager:', error)
    }
  }

  const deletePromptManager = async (id: number) => {
    try {
      await logicalDeletePromptManagerAction(id)
      removePromptManager(id)
    } catch (error) {
      // TODO react notificationを追加してエラー時にトーストを表示するようにする
      console.error('Failed to logical delete prompt manager:', error)
    }
  }

  const removePromptManager = (id: number) => {
    setPromptManagers(promptManagers.filter((item) => item.id !== id))
  }

  return (
    <div className="p-4">
      <SideMenuHeader
        onClickAdd={addTemporaryPromptManager}
        onClickConfig={goToConfigPage}
      />
      <PromptManagerList
        promptManagers={promptManagers}
        onClickPromptManager={selectPromptManager}
        onClickDeletePromptManager={deletePromptManager}
        handleSavePromptManager={savePromptManager}
        isVisibleNewManagerForm={isVisibleNewManagerForm}
        setIsVisibleNewManagerForm={setIsVisibleNewManagerForm}
      />
    </div>
  )
}

export default SideMenu
