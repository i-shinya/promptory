import { useState } from 'react'
import PromptManagerList from '../../features/prompt-manager/components/PromptManagerList'
import SideMenuHeader from './SideMenuHeader'
import { useNavigate } from 'react-router-dom'

const SideMenu = () => {
  const navigate = useNavigate()

  const [isVisibleNewManagerForm, setIsVisibleNewManagerForm] = useState(false)

  const goToConfigPage = () => {
    navigate('/config')
  }

  const addTemporaryPromptManager = () => {
    setIsVisibleNewManagerForm(true)
  }

  return (
    <div className="flex flex-col h-full p-4">
      <SideMenuHeader
        onClickAdd={addTemporaryPromptManager}
        onClickConfig={goToConfigPage}
      />
      <div className="overflow-y-scroll">
        <PromptManagerList
          isVisibleNewManagerForm={isVisibleNewManagerForm}
          setIsVisibleNewManagerForm={setIsVisibleNewManagerForm}
        />
      </div>
    </div>
  )
}

export default SideMenu
