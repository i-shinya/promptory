import { ReactNode } from 'react'
import SideMenu from './components/SideMenu'

type Props = { children: ReactNode }

const DefaultLayout = ({ children }: Props) => {
  return (
    <div className="flex dark:bg-zinc-950">
      <div className="w-72 h-screen dark:bg-zinc-950">
        <SideMenu />
      </div>
      <div className="flex-grow p-6 dark:bg-zinc-800">{children}</div>
    </div>
  )
}

export default DefaultLayout
