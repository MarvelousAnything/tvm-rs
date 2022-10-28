// A modified version of tranquility

let mem = []
mem.length = 65536
let sp // Stack Pointer. Points the the current top of the stack.
let fp // Frame pointer.
let hwin
let imgnum, butnum, labnum, tabnum
let edata
let debug = false
let pause = false
let m

startvm().then(r => console.log("done"))

function togglepause() {
    pause = !pause
}

function checkpause() {
    if (pause) {
        window.setTimeout(checkpause, 100)
    }
}

function push(x) {
    mem[sp] = x
    sp--
    console.log("PUSH " + x)
    updateviz()
}

function pop(x) {
    sp++
    updateviz()
    console.log("POP " + mem[sp])
    return mem[sp]
}

function a2s(x) {
    let s = ""
    while(mem[x] !== 0) {
        s += String.fromCharCode(mem[x])
        x++
    }
    return s
}

async function leval(l) {
    let pc = 0
    while(pc >= 0) {
        pc = await eval(l, pc)
    }
    return pc
}


async function call(n) {
    checkpause()
    await new Promise(r => setTimeout(r, 500));
    // Convert the old numbering to the new
    if(n > -100) {
        switch(n) {
            case -1: n = -101; break;
            case -2: n = -102; break;
            // case -3: n = -103: break;
            // case -4: n = -104; break;
            case -5: n = -105; break;
            case -10: n = -106; break;
            case -11: n = -107; break;
            case -12: n = -108; break;
            case -13: n = -201; break;
            case -14: n = -202; break;
            case -15: n = -203; break;
            case -16: n = -204; break;
            case -17: n = -205; break;
            case -18: n = -206; break;
            case -19: n = -109; break;
            case -20: n = -110; break;
            case -21: n = -111; break;
            case -22: n = -207; break;
            case -23: n = -208; break;
            case -24: n = -209; break;
            case -25: n = -103; break;
            case -26: n = -104; break;
            case -27: n = -210; break;
        }
    }
    if(n < -200 && hwin == null) {
        hwin = window.open()
    }
    switch(n) {
        case -101:	/* iprint */
            x = pop()
            console.log("Printing: " + x)
            stdout.value += x
            push(0)
            break
        case -102:	/* sprint */
            x = pop()
            console.info("CALL sprint " + x)
            s = a2s(x)
            stdout.value += s
            push(0)
            break
        case -103:	/* iread */
            p = pop()
            if(p === 0-1) {
                x = window.prompt("Integer input:")
            }
            else {
                s = a2s(p)
                x = window.prompt(s)
            }
            push(parseInt(x, 10))
            break
        case -104:	/* sread */
            a = pop()
            p = pop()
            if(p === -1) {
                x = window.prompt("String input:")
            }
            else {
                s = a2s(p)
                x = window.prompt(s)
            }
            for(i = 0; i < x.length; i++)
                mem[a+i] = x.charCodeAt(i)
            updateviz()
            mem[a+i] = 0
            updateviz()
            push(0)
            break
        case -105:	/* nl */
            stdout.value += "\n"
            console.info("CALL nl")
            push(0)
            break
        case -106:	/* random */
            n = pop()
            n = Math.floor(Math.random() * n)
            push(n)
            break
        case -107:	/* timer */
            var f
            to = pop()
            f = pop()
            n = setTimeout(function(){call(f)}, to)
            push(n)
            break
        case -108:	/* stoptimer */
            n = pop()
            clearTimeout(n)
            push(0)
            break
        case -201:	/* makeimg */
            hwin.document.write('<img id=img' + imgnum + ' />\n')
            push(imgnum)
            imgnum++
            break
        case -202:	/* setimg */
            n = pop()
            src = pop()
            s = a2s(src)
            i = hwin.document.getElementById('img'+n)
            i.src = s
            push(0)
            break
        case -203:	/* button */
            butname = pop()
            n = pop()
            s = a2s(butname)
            t1 = '<button id=but' + butnum + ' onclick="window.opener.call(' + n + ');">'
            hwin.document.write(t1 + s + '</button>\n')
            push(butnum)
            butnum++
            break
        case -204:	/* html */
            x = pop()
            s = a2s(x)
            hwin.document.write(s)
            push(0)
            break
        case -205: /* makelabel */
            labtxt = pop()
            s = a2s(labtxt)
            hwin.document.write('<label id=lab' + labnum + '>' + s + '</label>\n')
            push(labnum)
            labnum++
            break
        case -206:	/* setlabel */
            n = pop()
            label = pop()
            s = a2s(label)
            l = hwin.document.getElementById('lab'+n)
            l.innerHTML = s
            push(0)
            break
        case -109:	/* alloc */
            n = pop()
            push(edata)
            edata += n
            break
        case -110:	/* free */
            a = pop()
            push(0)
            break
        case -111:	/* i2s */
            s = pop()
            n = pop()
            istr = n.toString(10)
            for(i = 0; i < istr.length; i++)
                mem[s + i] = istr.charCodeAt(i)
            updateviz()
            mem[s+i] = 0
            updateviz()
            push(0)
            break
        case -207:	/* maketable */
            r = pop()
            c = pop()
            f = pop()
            hwin.document.write('<table id=tab' + tabnum + '>\n')
            for(i = 0; i < r; i++) {
                hwin.document.write('<tr>\n')
                for(j = 0; j < c; j++) {
                    hwin.document.write('<td onclick="window.opener.cellclick(' + tabnum + ',' + i + ',' + j + ',' + f + ')"></td>\n')
                }
                hwin.document.write('</tr>\n')
            }
            hwin.document.write('</table>\n')
            push(tabnum)
            tabnum++
            break
        case -208:	/* setcell */
            tnum = pop()
            r = pop()
            c = pop()
            cval = pop()
            s = a2s(cval)
            t = hwin.document.getElementById('tab' + tnum)
            t.rows[r].cells[c].innerHTML = s
            push(0)
            break
        case -209:	/* setcellcolor */
            tnum = pop()
            r = pop()
            c = pop()
            cval = pop()
            s = a2s(cval)
            t = hwin.document.getElementById('tab' + tnum)
            t.rows[r].cells[c].style = "background-color:" + s
            push(0)
            break
        case -210:	/* buttonlabel */
            bnum = pop()
            nlab = pop()
            s = a2s(nlab)
            b = hwin.document.getElementById('but' + bnum)
            b.innerHTML=s
            push(0)
            break
        case -151:  /* getkeydown */
            const keyPromise = () => new Promise(resolve => {
                addEventListener('keydown', resolve, {once:true, capture:true})
            })
            k = await keyPromise()
            console.log(k.keyCode)
            push(k.keyCode)
            break
        case -152:  /* ilog */
            x = pop()
            console.log(x)
            push(0)
            break
        case -153:  /* slog */
            x = pop()
            console.log(a2s(x))
            push(0)
            break
        case -3:	/* old iread */
            x = window.prompt("Integer input:")
            push(parseInt(x, 10))
            break
        case -4:	/* old sread */
            a = pop()
            x = window.prompt("String input:")
            for(i = 0; i < x.length; i++)
                mem[a+i] = x.charCodeAt(i)
            mem[a+i] = 0
            push(0)
            break
        default:
            if(n < 0) {
                console.log("Invalid function call", n)
                return
            }
            for(i = 0; i < m[n+2][3]; i++)
                push(0)
            mem[sp] = fp
            updateviz()
            fp = sp
            sp--
            updateviz()
            await leval(m[n+2][4])
            r = pop()
            sp = fp
            updateviz()
            fp = mem[sp]
            sp += m[n+2][2] + m[n+2][3]
            updateviz()
            push(r)
            break
    }
}

async function cellclick(t, r, c, f) {
    push(c)
    push(r)
    for(let i = 0; i < m[f+2][3]; i++)
        push(0)
    mem[sp] = fp
    fp = sp
    sp--
    await leval(m[f+2][4])
    r = pop()
    sp = fp
    fp = mem[sp]
    sp += m[f+2][2] + m[f+2][3]
    push(r)
}

async function eval(l, pc) {
    checkpause()
    await new Promise(r => setTimeout(r, 500));
    console.log("EVAL " + l[pc])
    if(pc >= l.length) {
        return -1
    }
    let ir = l[pc] // ir = instruction register?
    pc++ // increments the program counter to act as the input for the operation
    switch(ir) {
        case 1:	/* push */
            console.debug("PUSH " + l[pc])
            push(l[pc])
            pc++
            break
        case 2:	/* fetch */
            console.debug("FETCH " + l[pc])
            a = pop()
            push(mem[a])
            break
        case 3:	/* store */
            a = pop()
            v = pop()
            mem[a] = v
            console.debug("STORE " + a + " : " + v)
            break
        case 4:	/* if */
            x = pop()
            if(x != 0) {
                r = await leval(l[pc])
                pc += 2 // pc incremented by 2 because if requires 2 pointers to branches.
            }
            else {
                pc++
                r = await leval(l[pc])
                pc++
            }
            if(r <= -2)
                return r
            break
        case 5:	/* loop */
            console.debug("LOOP")
            while(1) {
                r = await leval(l[pc])
                if(r == -2)
                    break
                else if(r == -3)
                    return -3
            }
            pc++
            break
        case 6:	/* break */
            x = pop()
            if(x != 0)
                return -2
            break
        case 7:	/* return */
            return -3
            break
        case 8:	/* call */
            // console.debug("CALL " + l[pc])
            await call(l[pc])
            pc++
            break
        case 9:	/* fpplus */
            a = pop()
            // console.debug("FPPLUS " + a + " + " + fp)
            a += fp
            push(a)
            break
        case 10:	/* add */
            y = pop()
            x = pop()
            push(x+y)
            break
        case 11:	/* sub */
            y = pop()
            x = pop()
            push(x-y)
            break
        case 12:	/* mul */
            y = pop()
            x = pop()
            push(x*y)
            break
        case 13:	/* div */
            y = pop()
            x = pop()
            push(Math.floor(x/y))
            break
        case 14:	/* mod */
            y = pop()
            x = pop()
            push(x%y)
            break
        case 15:	/* not */
            x = pop()
            push(~x)
            break
        case 16:	/* and */
            y = pop()
            x = pop()
            push(x&y)
            break
        case 17:	/* or */
            y = pop()
            x = pop()
            push(x|y)
            break
        case 18:	/* xor */
            y = pop()
            x = pop()
            push(x^y)
            break
        case 19:	/* eq */
            y = pop()
            x = pop()
            if(x == y)
                push(1)
            else
                push(0)
            break
        case 20:	/* neq */
            y = pop()
            x = pop()
            if(x != y)
                push(1)
            else
                push(0)
            break
        case 21:	/* lt */
            y = pop()
            x = pop()
            if(x < y)
                push(1)
            else
                push(0)
            break
        case 22:	/* leq */
            y = pop()
            x = pop()
            if(x <= y)
                push(1)
            else
                push(0)
            break
        case 23:	/* gt */
            y = pop()
            x = pop()
            if(x > y)
                push(1)
            else
                push(0)
            break
        case 24:	/* geq */
            y = pop()
            x = pop()
            if(x >= y)
                push(1)
            else
                push(0)
            break
        case 25:	/* pop */
            pop()
            break
        case 26:	/* lshift */
            s = pop()
            x = pop()
            push(x << s)
            break
        case 27:	/* rshift */
            s = pop()
            x = pop()
            push(x >> s)
            break
        case 29:
            if (debug) debugger; console.debug("DEBUG");
            break
        default:
            console.log("unknown opcode ", ir)
            break
    }
    return pc
}

function vizmem() {
    let output = `<table class="mem"><thead><th>Address</th><th>Value</th></thead><tbody>`
    for(let i = 0; i < mem.length; i++) {
        if (mem[i] != undefined) {
            output += `<tr><td>${i}</td><td>${mem[i]}</td></tr>`
        }
    }
    output += "</tbody></table>"
    return output
}

function updateviz() {
    let body = document.querySelector('.mem').getElementsByTagName('tbody')[0]
    body.innerHTML = ""

    for(let i = 0; i < mem.length; i++) {
        if (mem[i] !== undefined) {
            let text = `<tr><td>${i}</td><td>${mem[i]}</td></tr>`
            if (i === sp) {
                text = `<tr><td style="background-color:aquamarine">${i}</td><td>${mem[i]}</td></tr>`
            }
            if (i === fp) {
                text = `<tr><td style="background-color:darksalmon">${i}</td><td>${mem[i]}</td></tr>`
            }
            body.innerHTML += text
        }
    }
}

async function start() {
    edata = m[0][1]
    await call(m[0][0])
    console.log("halt")
}

async function startvm() {
    hwin = null
    imgnum = 0
    butnum = 0
    labnum = 0
    tabnum = 0
    document.write("<html lang='en'>\n<body>\n")
    document.write('<button onclick="start();">Start</button><br>\n')
    document.write('<button onclick="togglepause();">Pause</button><br>\n')
    document.write("<label>Standard Output</label><br>\n")
    document.write('<textarea id="stdout" cols="80" rows="20"></textarea><br>\n')
    document.write(vizmem())
    document.write("</body>\n</html>\n")
    m = JSON.parse(tape)
    sp = 65535
    fp = 65535

    /* Bootstrapper; Loads tape to m which contains a few different sections.
        0: Allocates the memory necessary for the program in section 1. Also defines the first call of the program.
        1: The "main" section. Writes instructions to the memory allocated in section 0.
        2...: Where functions are defined.*/
    for(const element of m[1]) {
        mem[element[0]] = element[1]
    }
}