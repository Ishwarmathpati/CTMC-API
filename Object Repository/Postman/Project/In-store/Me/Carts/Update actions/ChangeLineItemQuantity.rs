<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>ChangeLineItemQuantity</name>
   <tag></tag>
   <elementGuidId>fed17c72-c9ef-4547-88ab-69c7d13a8fff</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;version\&quot;: ${cart-version},\n    \&quot;actions\&quot;: [\n        {\n            \&quot;action\&quot; : \&quot;changeLineItemQuantity\&quot;,\n            \&quot;lineItemId\&quot; : \&quot;${lineItemId}\&quot;,\n            \&quot;quantity\&quot; : 1\n          }\n    ]\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer {{ctp_access_token}}</value>
      <webElementGuid>a77e3ba1-c61b-4528-97f5-d80d90366808</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>407483b8-bc66-4272-9480-7c23ac7692f5</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.9</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${host}/${project-key}/in-store/key=${store-key}/me/carts/${cart-id}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.host</defaultValue>
      <description></description>
      <id>8864fb2b-2b84-4bfb-8dbd-6c4809873af5</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project-key</defaultValue>
      <description></description>
      <id>65340fd7-c2f1-46b2-809c-38ac2c8d5dfb</id>
      <masked>false</masked>
      <name>project-key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.store-key</defaultValue>
      <description></description>
      <id>f99974fd-2a84-49dd-8d9e-899a332564c8</id>
      <masked>false</masked>
      <name>store-key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.cart-id</defaultValue>
      <description></description>
      <id>3717c191-d595-4a72-a489-6d9731382953</id>
      <masked>false</masked>
      <name>cart-id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.cart-version</defaultValue>
      <description></description>
      <id>12a14878-c318-4003-8596-5d023e474a9c</id>
      <masked>false</masked>
      <name>cart-version</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.lineItemId</defaultValue>
      <description></description>
      <id>68b5d204-9899-47bc-91e6-d0d7e8ffb18a</id>
      <masked>false</masked>
      <name>lineItemId</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
