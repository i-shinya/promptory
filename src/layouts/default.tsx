import { ReactNode } from 'react'
import SideMenu from './components/SideMenu'
import { ToastContainer } from 'react-toastify'
import 'react-toastify/dist/ReactToastify.css'

type Props = { children: ReactNode }

const DefaultLayout = ({ children }: Props) => {
  return (
    <div className="flex flex-row dark:bg-zinc-950 h-screen max-h-screen">
      <div className="flex flex-col w-72 min-w-72 dark:bg-zinc-950">
        <SideMenu />
      </div>
      <div className="flex flex-col p-2 grow dark:bg-zinc-800 h-full max-h-full">
        {children}
      </div>
      <ToastContainer
        position="top-right"
        autoClose={4000}
        hideProgressBar={false}
        newestOnTop={false}
        closeOnClick
        rtl={false}
        pauseOnFocusLoss
        draggable
        pauseOnHover
        theme="dark"
      />
    </div>
  )
}

export default DefaultLayout
