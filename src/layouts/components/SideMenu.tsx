import PromptManagerList from '../../features/prompt-manager/components/PromptManagerList'
import { PromptManager } from '../../features/prompt-manager/types'
import SideMenuHeader from './SideMenuHeader'
import { useNavigate } from 'react-router-dom'

const SideMenu = () => {
  const navigate = useNavigate()

  // TODO rust側から取得する
  const promptManagers: PromptManager[] = [
    { id: 1, title: 'Chat APIお試し', apiType: 'Chat', tags: ['test', 'chat'] },
    { id: 2, title: 'Assistant APIお試し', apiType: 'Assistant', tags: [] },
    {
      id: 3,
      title: 'Assistant APIお試し２',
      apiType: 'Assistant',
      tags: ['test', 'sample'],
    },
  ]

  const selectPromptManager = (id: number) => {
    // TODO詳細画面を作成したらそちらに遷移するようにする
    console.log(id)
    navigate('/prompt_manager')
  }

  const goToConfigPage = () => {
    navigate('/config')
  }

  const addPromptManager = () => {
    // TODO rust側の追加処理を呼び出す
    console.log('add')
  }

  const deletePromptManager = (id: number) => {
    // TODO rust側の削除処理を呼び出す
    console.log(id)
  }

  return (
    <div className="w-64 h-screen dark:bg-zinc-950 p-4">
      <SideMenuHeader
        onClickAdd={addPromptManager}
        onClickConfig={goToConfigPage}
      />
      <PromptManagerList
        promptManagers={promptManagers}
        onClickPromptManager={selectPromptManager}
        onClickDeletePromptManager={deletePromptManager}
      />
    </div>
  )
}

export default SideMenu
