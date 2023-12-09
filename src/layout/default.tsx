import { ReactNode } from 'react'

type Props = { children: ReactNode }

const Layout = ({ children }: Props) => {
  return (
    <div>
      <div>{children}</div>
    </div>
  )
}

export default Layout
