import { ReactNode } from 'react'

type CardProps = { children: ReactNode }

const Card = ({ children }: CardProps) => {
  return <div className="dark:bg-zinc-800 p-2 m-2 rounded">{children}</div>
}

export default Card
