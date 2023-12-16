import { ReactNode } from 'react'
import SideMenu from './components/SideMenu'
import { ToastContainer } from 'react-toastify'
import 'react-toastify/dist/ReactToastify.css'

type Props = { children: ReactNode }

const DefaultLayout = ({ children }: Props) => {
  return (
    <div className="flex dark:bg-zinc-950">
      <div className="w-72 h-screen dark:bg-zinc-950">
        <SideMenu />
      </div>
      <div className="flex-grow p-6 dark:bg-zinc-800">{children}</div>
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
