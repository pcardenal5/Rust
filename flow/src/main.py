def fibo(n :int, previous: dict[int,int]) :
    if n<=2:
        previous = {
            1:1,
            2:1
        }
        return (n, previous)
    
    else:
        if previous.get(n):
            return (n, previous)

        else:
            fN2 = fibo(n-2, previous)
            fN1 = fibo(n-1, fN2[1])
            previous[n] = fN1[1][n-1] + fN1[1][n-2]
            return (n, previous)

def main():
    previous = {}
    for i in range(1, 185):
        result = fibo(i,previous)
        previous = result[1]
        # print(f"El número de fibonacci en la posición {i} es {previous[i]}")

if __name__ == '__main__':
    main()