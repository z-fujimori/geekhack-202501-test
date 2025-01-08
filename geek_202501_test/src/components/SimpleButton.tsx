const SimpleButton = (props:{
    text: string,
    function: () => Promise<void>
}) => {
    return (
        <div>
            <h3>{props.text}</h3>
            <button onClick={props.function}>Click</button>
        </div>
    )
}

export default SimpleButton