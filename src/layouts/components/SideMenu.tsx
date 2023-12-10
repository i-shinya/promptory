import { useState } from 'react'
import PromptManagerList from '../../features/prompt-manager/components/PromptManagerList'
import { PromptManager } from '../../features/prompt-manager/types'
import SideMenuHeader from './SideMenuHeader'
import { useNavigate } from 'react-router-dom'
import { createPromptManagerAction } from '../../features/prompt-manager/actions'

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
    // TODO詳細画面を作成したらそちらに遷移するようにする
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
    if (title === '') {
      return
    }
    try {
      const id = await createPromptManagerAction(title)
      console.log(id)
      setIsVisibleNewManagerForm(false)
      setPromptManagers([
        ...promptManagers,
        { id, title, apiType: null, tags: [] },
      ])
    } catch (error) {
      // TODO react notificationを追加してエラー時に表示する
      console.error('Failed to create prompt manager:', error)
    }
  }

  const deletePromptManager = (id: number) => {
    // TODO rust側の削除処理を呼び出す
    console.log(`delete ${id}`)
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
        handleSavePropmtManager={savePromptManager}
        isVisibleNewManagerForm={isVisibleNewManagerForm}
        setIsVisibleNewManagerForm={setIsVisibleNewManagerForm}
      />
    </div>
  )
}

export default SideMenu
