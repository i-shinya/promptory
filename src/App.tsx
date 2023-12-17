import { Routes, Route } from 'react-router-dom'
import './App.css'
import Config from './pages/Config'
import PromptManager from './pages/PromptManager'
import DefaultLayout from './layouts/default'
import { RecoilRoot } from 'recoil'
import { ThemeProvider } from './layouts/theme-provider'
import Home from './pages/Home'

const App = () => {
  return (
    <ThemeProvider defaultTheme="dark" storageKey="vite-ui-theme">
      <RecoilRoot>
        <DefaultLayout>
          <Routes>
            <Route path="/" element={<Home />} />
            <Route path="/prompt_manager/:id" element={<PromptManager />} />
            <Route path="/config" element={<Config />} />
          </Routes>
        </DefaultLayout>
      </RecoilRoot>
    </ThemeProvider>
  )
}

export default App
