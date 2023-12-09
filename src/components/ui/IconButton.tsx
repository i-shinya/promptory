export interface IconButtonProps {
  icon: string
  onClick: () => void
}

const IconButton = ({ icon, onClick }: IconButtonProps) => {
  return (
    <button
      className={`${icon} cursor-pointer p-0`}
      onClick={(e: any) => {
        e.stopPropagation()
        onClick()
      }}
    ></button>
  )
}

export default IconButton
