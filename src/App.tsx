import { Routes, Route } from 'react-router-dom'
import './App.css'
import Config from './pages/Config'
import PromptManager from './pages/PromptManager'
import Layout from './layout/default'

function App() {
  return (
    <Layout>
      <Routes>
        <Route path="/" element={<PromptManager />} />
        <Route path="/prompt_manager" element={<PromptManager />} />
        <Route path="/config" element={<Config />} />
      </Routes>
    </Layout>
  )
}

export default App
