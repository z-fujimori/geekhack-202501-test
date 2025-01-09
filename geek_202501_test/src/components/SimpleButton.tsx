const SimpleButton = (props:{
    text: string,
    function: () => Promise<void>
}) => {
    return (
        <div>
            <h3>{props.text}</h3>
            <button className="p-2 rounded-lg bg-amber-600" onClick={props.function}>Click</button>
        </div>
    )
}

export default SimpleButton