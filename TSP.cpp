#include<stdio.h>
#include<stdlib.h>
#include<time.h>
double dis[24][24]={0},d[24][1<<23]={0},p[24][1<<23]={0};
int main()
{
  int n;
    double temp,min;
    clock_t start,finish;
    double Total_time;
    scanf("%d",&n);
    for(int i=0;i<n;i++)
    {
        for(int j=0;j<n;j++)
        {
            scanf("%lf",&dis[i][j]);
            if(dis[i][j]==-1) dis[i][j]=99999999;
        }
    }
    start=clock();
    for(int i=1;i<n;i++)
    {
        d[i][0]=dis[i][0];
    }
    for(int j=1;j<(1<<(n-1));j++)
    {
        for(int i=1;i<n;i++)
        {
            if(((1<<(i-1))&j)==0)
            {
                min=99999999;
                for(int k=1;k<n;k++)
                {
                    if((1<<(k-1))&j)
                    {
                        temp=dis[i][k]+d[k][j-(1<<(k-1))];
                        if(temp<min)
                        {
                            min=temp;
                            d[i][j]=temp;
                            p[i][j]=k;
                        }
                    }
                }
            }
        }
    }
    min=99999999;
    for(int k=1;k<n;k++)
    {
        temp=dis[0][k]+d[k][((1<<(n-1))-1)-(1<<(k-1))];
        if(min>temp)
        {
            min=temp;
            d[0][(1<<(n-1))-1]=temp;
            p[0][(1<<(n-1))-1]=k;
        }
    }
    printf("%.16lf\n", min);
    return 0;
    printf("min value is: %lf\n",min);
    int temp1;
    temp1=(1<<(n-1))-1;
    printf("the route is: 0-");
    int j=p[0][temp1];
    while(j!=0)
    {
        printf("%d-",j);
        temp1-=(1<<(j-1));
        j=p[j][temp1];
    }
    finish=clock();
    Total_time=(double)(finish-start)/CLOCKS_PER_SEC;
    printf("0\n");
    printf("Total time: %.4lfs\n",Total_time);
//    system("pause");
    return 0;
}
/*
4
0 2 9 10
1 0 6 4
15 7 0 8
6 3 12 0

4
0 3 6 7
5 0 2 3
6 4 0 2
3 7 5 0*/