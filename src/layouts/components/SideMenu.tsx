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
    <div className="p-4">
      <SideMenuHeader
        onClickAdd={addTemporaryPromptManager}
        onClickConfig={goToConfigPage}
      />
      <PromptManagerList
        isVisibleNewManagerForm={isVisibleNewManagerForm}
        setIsVisibleNewManagerForm={setIsVisibleNewManagerForm}
      />
    </div>
  )
}

export default SideMenu
