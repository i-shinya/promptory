import { ReactNode } from 'react'
import SideMenu from './components/SideMenu'

type Props = { children: ReactNode }

const DefaultLayout = ({ children }: Props) => {
  return (
    <div className="flex">
      <SideMenu />
      <div className="flex-grow p-8">{children}</div>
    </div>
  )
}

export default DefaultLayout
