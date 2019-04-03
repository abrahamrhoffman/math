
class PrimeFinder(object):

    def find(self, number):
        if isinstance(number, int):
            pass
        else:
            print("Number must be of type: integer")
            raise SystemExit

        if number <= 0:
            print("A non-negative, non-zero integer is required")
            raise SystemExit
        
        if number == 1:
            print(1)

        for n in range(2, number):
            for n in range(2, number):
                


        print(aList)

def main():
    pf = PrimeFinder()
    pf.find(10)    

if __name__ == "__main__":
    main()