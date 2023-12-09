import { Routes, Route } from 'react-router-dom'
import './App.css'
import Config from './pages/Config'
import PromptManager from './pages/PromptManager'
import DefaultLayout from './layouts/default'

const App = () => {
  return (
    <DefaultLayout>
      <Routes>
        <Route path="/" element={<PromptManager />} />
        <Route path="/prompt_manager" element={<PromptManager />} />
        <Route path="/config" element={<Config />} />
      </Routes>
    </DefaultLayout>
  )
}

export default App
