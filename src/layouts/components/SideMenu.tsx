import { useState } from 'react'
import PromptManagerList from '../../features/prompt-manager/components/PromptManagerList'
import { PromptManager } from '../../features/prompt-manager/types'
import SideMenuHeader from './SideMenuHeader'
import { useNavigate } from 'react-router-dom'
import {
  createPromptManagerAction,
  logicalDeletePromptManagerAction,
} from '../../features/prompt-manager/actions'

const SideMenu = () => {
  const navigate = useNavigate()

  const [isVisibleNewManagerForm, setIsVisibleNewManagerForm] = useState(false)

  // TODO rust側から取得する
  const [promptManagers, setPromptManagers] = useState<PromptManager[]>([
    { id: 1, title: 'Chat APIお試し', apiType: 'Chat', tags: ['test', 'chat'] },
    { id: 2, title: 'Assistant APIお試し', apiType: 'Assistant', tags: [] },
    {
      id: 3,
      title: 'Assistant APIお試し2',
      apiType: 'Assistant',
      tags: ['test', 'sample'],
    },
  ])

  const selectPromptManager = (id: number) => {
    // TODO 詳細画面を作成したらそちらに遷移するようにする
    console.log(`select ${id}`)
    navigate('/prompt_manager')
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
        { id, title, apiType: null, tags: [] },
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
    <div className="w-64 h-screen dark:bg-zinc-950 p-4">
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
